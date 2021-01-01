open Batteries

type damage_type = Fire | Cold | Radiation | Slashing | Bludgeoning

let damage_type_from_string s =
  match s with
  | "fire" -> Fire
  | "cold" -> Cold
  | "radiation" -> Radiation
  | "slashing" -> Slashing
  | "bludgeoning" -> Bludgeoning
  | _ -> failwith ("invalid damage type " ^ s)

let damage_type_to_string s =
  match s with
  | Fire -> "fire"
  | Cold -> "cold"
  | Radiation -> "radiation"
  | Slashing -> "slashing"
  | Bludgeoning -> "bludgeoning"

let damage_types_from_string s =
  s |> String.split_on_char ','
  |> List.map (fun s -> s |> String.trim |> damage_type_from_string)
  |> Set.of_list

type team = ImmuneSystem | Infection

let other_team t =
  match t with ImmuneSystem -> Infection | Infection -> ImmuneSystem

let team_to_string t =
  match t with ImmuneSystem -> "immune system" | Infection -> "infection"

type group =
  { id: int
  ; team: team
  ; nunits: int
  ; hp: int
  ; immunities: damage_type Set.t
  ; weaknesses: damage_type Set.t
  ; damage: int
  ; damage_type: damage_type
  ; initiative: int }

let damage_types_to_string dts =
  dts |> Set.to_list |> List.map damage_type_to_string |> String.join ", "

let group_to_string g =
  Printf.sprintf
    "group %d: team %s, %d units each with %d hit points (immune to %s; weak \
     to %s) with an attack that does %d %s damage at initiative %d"
    g.id (team_to_string g.team) g.nunits g.hp
    (damage_types_to_string g.immunities)
    (damage_types_to_string g.weaknesses)
    g.damage
    (damage_type_to_string g.damage_type)
    g.initiative

let groups_to_string gs = gs |> List.map group_to_string |> String.join "\n"

let groups_by_id_to_string gs_by_id =
  gs_by_id |> Hashtbl.values |> List.of_enum
  |> List.sort (fun a b -> compare a.id b.id)
  |> groups_to_string

let effective_power g = g.nunits * g.damage

let effective_damage attacker defender =
  if Set.mem attacker.damage_type defender.immunities then 0
  else if Set.mem attacker.damage_type defender.weaknesses then
    attacker.damage * 2
  else attacker.damage

let attack_defender attacker defender =
  let ed = effective_damage attacker defender in
  let tot_damage = ed * attacker.nunits in
  let killed = min (tot_damage / defender.hp) defender.nunits in
  (* Printf.printf "%d attacks %d for %d damage, %d killed\n" attacker.id
    defender.id tot_damage killed ; *)
  {defender with nunits= defender.nunits - killed}

let enemy_groups groups team = List.filter (fun g -> g.team != team) groups

let select_targets groups_by_id =
  let groups = Hashtbl.values groups_by_id |> List.of_enum in
  (* In decreasing order of effective power, groups choose their targets; in a
     tie, the group with the higher initiative chooses first. *)
  let selection_order =
    List.sort
      (fun a b ->
        let ep_comp = compare (effective_power b) (effective_power a) in
        if ep_comp != 0 then ep_comp else compare b.initiative a.initiative )
      groups
  in
  let selections = Hashtbl.create 20 in
  List.iter
    (fun g ->
      let targets =
        enemy_groups groups g.team
        (* Defending groups can only be chosen as a target by one attacking group. *)
        |> List.filter (fun g ->
               not
                 ( selections |> Hashtbl.values
                 |> Enum.exists (fun id -> id = g.id) ) )
        |> List.sort (fun a b ->
               (* The attacking group chooses to target the group in the enemy army to
                  which it would deal the most damage (after accounting for weaknesses
                  and immunities, but not accounting for whether the defending group
                  has enough units to actually receive all of that damage). *)
               let damage_comp =
                 compare (effective_damage g b) (effective_damage g a)
               in
               if damage_comp != 0 then damage_comp
               else
                 (* If an attacking group is considering two defending groups to
                    which it would deal equal damage, it chooses to target the
                    defending group with the largest effective power *)
                 let ep_comp =
                   compare (effective_power b) (effective_power a)
                 in
                 (* If there is still a tie, it chooses the defending group with
                    the highest initiative *)
                 if ep_comp != 0 then ep_comp
                 else compare b.initiative a.initiative )
      in
      if not (List.is_empty targets) then
        let target = List.hd targets in
        (* If it cannot deal any defending groups damage, it does not choose a
           target. *)
        if effective_damage g target != 0 then
          Hashtbl.replace selections g.id target.id )
    selection_order ;
  selections

let process_attacks groups_by_id selections =
  (* Groups attack in decreasing order of initiative *)
  let ids_in_order =
    groups_by_id |> Hashtbl.values |> List.of_enum
    |> List.sort (fun a b -> compare b.initiative a.initiative)
    |> List.map (fun g -> g.id)
  in
  List.iter
    (fun attacker_id ->
      if
        (* Ensure the attacker is still alive. *)
        Hashtbl.mem groups_by_id attacker_id
        (* Ensure a target was selected *)
        && Hashtbl.mem selections attacker_id
      then
        let defender_id = Hashtbl.find selections attacker_id in
        (* Ensure that the defender is still alive. *)
        if Hashtbl.mem groups_by_id defender_id then
          let attacker = Hashtbl.find groups_by_id attacker_id in
          let defender = Hashtbl.find groups_by_id defender_id in
          let defender = attack_defender attacker defender in
          if defender.nunits = 0 then Hashtbl.remove groups_by_id defender_id
          else Hashtbl.replace groups_by_id defender_id defender )
    ids_in_order

let rec run_battle groups_by_id =
  (* groups_by_id |> groups_by_id_to_string |> print_endline ;
  print_newline () ; *)
  let is_ngroups =
    fold
      (fun accum group ->
        if group.team = ImmuneSystem then accum + 1 else accum )
      0
      (Hashtbl.values groups_by_id)
  in
  let infection_ngroups =
    fold
      (fun accum group -> if group.team = Infection then accum + 1 else accum)
      0
      (Hashtbl.values groups_by_id)
  in
  if is_ngroups = 0 || infection_ngroups = 0 then groups_by_id
  else
    let selections = select_targets groups_by_id in
    process_attacks groups_by_id selections ;
    run_battle groups_by_id

let remove_prefix s prefix =
  if String.starts_with s prefix then
    let a = String.length s in
    let b = String.length prefix in
    String.sub s b (a - b)
  else s

let parse_defense s =
  let chunks = s |> String.split_on_char ';' in
  List.fold_left
    (fun accum chunk ->
      let chunk = String.trim chunk in
      let immunities, weaknesses = accum in
      let immunities =
        if String.starts_with chunk "immune to" then
          remove_prefix chunk "immune to" |> damage_types_from_string
        else immunities
      in
      let weaknesses =
        if String.starts_with chunk "weak to" then
          remove_prefix chunk "weak to" |> damage_types_from_string
        else weaknesses
      in
      (immunities, weaknesses) )
    (Set.empty, Set.empty) chunks

let parse_line line id team =
  let a, b, c =
    if String.contains line '(' then (
      let re =
        Str.regexp
          "^\\([^(]+\\) (\\([^)]+\\)) with an attack that does \\(.+\\)$"
      in
      ignore (Str.search_forward re line 0) ;
      let a = Str.matched_group 1 line in
      let b = Str.matched_group 2 line in
      let c = Str.matched_group 3 line in
      (a, b, c) )
    else
      let re = Str.regexp "^\\(.+\\) with an attack that does \\(.+\\)$" in
      ignore (Str.search_forward re line 0) ;
      let a = Str.matched_group 1 line in
      let c = Str.matched_group 2 line in
      (a, "", c)
  in
  let nunits, hp =
    Scanf.sscanf a "%d units each with %d hit points" (fun nunits hp ->
        (nunits, hp) )
  in
  let immunities, weaknesses = parse_defense b in
  let damage, damage_type, initiative =
    Scanf.sscanf c "%d %s damage at initiative %d"
      (fun damage damage_type initiative ->
        (damage, damage_type_from_string damage_type, initiative) )
  in
  { id
  ; team
  ; nunits
  ; hp
  ; immunities
  ; weaknesses
  ; damage
  ; damage_type
  ; initiative }

let parse data =
  let lines = data |> String.trim |> String.split_on_char '\n' in
  let groups, _, _ =
    List.fold_left
      (fun accum line ->
        let groups, id, team = accum in
        let team =
          if String.exists line "Infection:" then Infection else team
        in
        let groups, id =
          if String.length line > 0 && not (String.contains line ':') then
            (parse_line line id team :: groups, id + 1)
          else (groups, id)
        in
        (groups, id, team) )
      ([], 0, ImmuneSystem) lines
  in
  groups

let groups_by_id groups =
  let groups_by_id = Hashtbl.create 20 in
  List.iter (fun g -> Hashtbl.replace groups_by_id g.id g) groups ;
  groups_by_id

let boost groups team amount =
  List.map
    (fun g -> if g.team = team then {g with damage= g.damage + amount} else g)
    groups

let find_boost groups =
  let rec aux n =
    dump n |> print_endline ;
    let groups = boost groups ImmuneSystem n in
    let groups_by_id = groups_by_id groups in
    let remaining =
      run_battle groups_by_id |> Hashtbl.values |> List.of_enum
    in
    if (List.hd remaining).team = ImmuneSystem then remaining else aux (n + 1)
  in
  aux 43

let example_data =
  {|
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
|}

let actual_data = IO.stdin |> IO.read_all

let data = actual_data

let () =
  let groups = parse data in
  let remaining = find_boost groups in
  Printf.printf "\nRemaining units: %d\n"
    (List.fold_left (fun accum g -> accum + g.nunits) 0 remaining)
