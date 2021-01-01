open Batteries

module Parser = struct
  type node = {name: string; weight: int; children: string list}

  let name_and_weight prefix = Scanf.sscanf prefix "%s (%d)" (fun n w -> (n, w))

  let rec make_nodemap nodes map =
    match nodes with
    | [] -> map
    | first :: rest ->
        map |> Map.String.add first.name first |> make_nodemap rest

  let parse src =
    let nodes =
      Str.split (Str.regexp "\n+") src
      |> List.map (fun line ->
             match Str.split (Str.regexp " -> ") line with
             | [prefix] ->
                 let name, weight = name_and_weight prefix in
                 {name; weight; children= []}
             | [prefix; children] ->
                 let name, weight = name_and_weight prefix in
                 {name; weight; children= Str.split (Str.regexp ", ") children}
             | _ -> failwith @@ "Invalid line: " ^ line )
    in
    make_nodemap nodes Map.String.empty
end

module Tree = struct
  type node = {name: string; weight: int; tree_weight: int; children: node list}

  let rec make (root_name: string) (nodemap: Parser.node Map.String.t) =
    let root_desc = Map.String.find root_name nodemap in
    if List.length root_desc.children = 0 then
      { name= root_desc.name
      ; weight= root_desc.weight
      ; tree_weight= root_desc.weight
      ; children= [] }
    else
      let children =
        List.map (fun name -> make name nodemap) root_desc.children
      in
      { name= root_desc.name
      ; weight= root_desc.weight
      ; tree_weight=
          root_desc.weight
          + List.fold_left (fun acc n -> acc + n.tree_weight) 0 children
      ; children }

  let nodes_by_tree_weight nodes =
    List.fold_left
      (fun acc n ->
        if Map.Int.mem n.tree_weight acc then
          Map.Int.add n.tree_weight (n :: Map.Int.find n.tree_weight acc) acc
        else Map.Int.add n.tree_weight [n] acc )
      Map.Int.empty nodes

  let get_imbalance root =
    let children_by_tree_weight = nodes_by_tree_weight root.children in
    let child_tree_weights =
      Map.Int.keys children_by_tree_weight |> Set.Int.of_enum
    in
    let imbalance_child =
      List.filter
        (fun n ->
          let num_at_weight =
            Map.Int.find n.tree_weight children_by_tree_weight |> List.length
          in
          num_at_weight = 1 )
        root.children
      |> List.first
    in
    let other_tree_weight =
      Set.Int.remove imbalance_child.tree_weight child_tree_weights
      |> Set.Int.to_list |> List.first
    in
    imbalance_child.tree_weight - other_tree_weight

  let rec find_bad_node imbalance root =
    let children_by_tree_weight = nodes_by_tree_weight root.children in
    let child_tree_weights =
      List.of_enum (Map.Int.keys children_by_tree_weight)
    in
    let fail_bad_weights () =
      failwith
        (Printf.sprintf
           "Node %s has invalid distribution of child weights: %s\n" root.name
           (dump child_tree_weights))
    in
    match List.length child_tree_weights with
    | 1 -> root
    | 2 ->
        if imbalance < 0 then (
          let min_children =
            Map.Int.find
              (min
                 (List.first child_tree_weights)
                 (List.last child_tree_weights))
              children_by_tree_weight
          in
          if List.length min_children <> 1 then fail_bad_weights () ;
          find_bad_node imbalance @@ List.first min_children )
        else
          let max_children =
            Map.Int.find
              (max
                 (List.first child_tree_weights)
                 (List.last child_tree_weights))
              children_by_tree_weight
          in
          if List.length max_children <> 1 then fail_bad_weights () ;
          find_bad_node imbalance (List.first max_children)
    | _ -> fail_bad_weights ()
end

let () =
  let tree = IO.stdin |> IO.read_all |> Parser.parse |> Tree.make "svugo" in
  let imbalance = Tree.get_imbalance tree in
  let bad_node = Tree.find_bad_node imbalance tree in
  Printf.printf "Bad node: %s, weight: %d, intended weight: %d\n" bad_node.name
    bad_node.weight
    (bad_node.weight - imbalance)
