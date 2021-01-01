require 'json'
require 'set'

def deep_dup(obj)
  case obj
  when Hash
    obj.reduce({}) do |memo, (k, v)|
      memo[k] = deep_dup(v)
      memo
    end
  when Array
    obj.reduce([]) do |memo, v|
      memo << deep_dup(v)
      memo
    end
  else
    obj
  end
end

class State
  def initialize(state = {})
    @state = state

    floors.each do |floor|
      floor['generators'].sort!
      floor['microchips'].sort!
    end
  end

  def dup
    self.class.new(deep_dup(@state))
  end

  def serialize
    x = deep_dup(@state)
    material_map = {}
    unmatched = 0
    x['floors'].each do |floor|
      floor['generators'].each do |material|
        next if material_map.key?(material)
        if floor['microchips'].include?(material)
          material_map[material] = 'matched'
        else
          material_map[material] = "unmatched #{unmatched}"
          unmatched += 1
        end
      end
    end

    x['floors'].each do |floor|
      floor['generators'].map! { |m| material_map[m] }.sort!
      floor['microchips'].map! { |m| material_map[m] }.sort!
    end

    x.to_json
  end

  def to_h
    @state
  end

  def floors
    @state['floors']
  end

  def elevator
    @state['elevator']
  end

  def current_floor
    floors[elevator]
  end

  def next_moves
    combos = movable_combinations

    next_floors = nil
    case elevator
    when 0 then next_floors = [1]
    when 1 then next_floors = [0, 2]
    when 2 then next_floors = [1, 3]
    when 3 then next_floors = [2]
    end

    res = []
    combos.each do |c|
      next_floors.each do |f|
        s = dup.apply!(c, f)
        res << [c, f, s] if s.legal?
      end
    end
    res
  end

  def apply!(delta, new_floor)
    ['generators', 'microchips'].each do |type|
      (delta[type] || []).each do |material|
        floors[elevator][type].delete(material)
        floors[new_floor][type] << material
      end
    end

    @state['elevator'] = new_floor

    self
  end

  def legal?
    floors.each do |floor|
      next if floor['generators'].empty?
      next if floor['microchips'].empty?

      floor['microchips'].each do |material|
        return false unless floor['generators'].include?(material)
      end
    end

    true
  end

  def movable_combinations
    res = []

    current_floor['generators'].each do |material|
      next unless current_floor['microchips'].include?(material)
      res << {'generators' =>[material], 'microchips' =>[material]}
    end

    ['generators', 'microchips'].each do |type|
      [1, 2].each do |size|
        current_floor[type].combination(size).each do |combo|
          res << {type => combo}
        end
      end
    end

    res
  end

end

initial_state = State.new({
  'elevator' => 0,
  'floors' => [
    {
      'generators' =>['strontium', 'plutonium', 'elerium', 'dilithium'],
      'microchips' =>['strontium', 'plutonium', 'elerium', 'dilithium'],
    },
    {
      'generators' =>['ruthenium', 'curium', 'thulium'],
      'microchips' =>['ruthenium', 'curium'],
    },
    {
      'generators' =>[],
      'microchips' =>['thulium'],
    },
    {
      'generators' =>[],
      'microchips' =>[],
    }
  ],
})

final_state = State.new({
  'elevator' => 3,
  'floors' => [
    {
      'generators' =>[],
      'microchips' =>[],
    },
    {
      'generators' =>[],
      'microchips' =>[],
    },
    {
      'generators' =>[],
      'microchips' =>[],
    },
    {
      'generators' =>['curium', 'strontium', 'plutonium', 'thulium', 'ruthenium', 'elerium', 'dilithium'],
      'microchips' =>['curium', 'strontium', 'plutonium', 'thulium', 'ruthenium', 'elerium', 'dilithium'],
    }
  ],
})

final_key = final_state.serialize
earliest_arrival = {}

q = [{
  state: initial_state,
  moves: 0,
  path: [],
}]

loop do
  node = q.shift # BFS

  k = node[:state].serialize

  if k == final_key
    puts node[:moves]
    puts node[:path].inspect
    exit
  end

  if earliest_arrival[k] && earliest_arrival[k] <= node[:moves]
    next
  end

  earliest_arrival[k] = node[:moves]

  node[:state].next_moves.each do |(c, f, next_state)|
    q << {
      state: next_state,
      moves: node[:moves] + 1,
      path: node[:path].dup + [c, f],
    }
  end
end
