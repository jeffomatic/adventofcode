require 'digest'

def available_dirs(passcode, path, location)
  hash = Digest::MD5.hexdigest("#{passcode}#{path}")

  hash_dirs = hash[0...4].each_char.map do |c|
    ['b', 'c', 'd', 'e', 'f'].include?(c)
  end

  res = []
  res << 'U' if hash_dirs[0] && 0 < location[1]
  res << 'D' if hash_dirs[1] && location[1] < 3
  res << 'L' if hash_dirs[2] && 0 < location[0]
  res << 'R' if hash_dirs[3] && location[0] < 3
  res
end

q = [{path: '', location: [0, 0]}]
passcode = 'edjrjqaa'

loop do
  room = q.shift
  dirs = available_dirs(passcode, room[:path], room[:location])

  if room[:location] == [2, 3] && dirs.include?('R')
    puts room[:path] + 'R'
    exit
  elsif room[:location] == [3, 2] && dirs.include?('D')
    puts room[:path] + 'D'
    exit
  end

  dirs.each do |d|
    new_room = {
      path: room[:path] + d,
      location: room[:location].dup
    }

    case d
    when 'U' then new_room[:location][1] -= 1
    when 'D' then new_room[:location][1] += 1
    when 'L' then new_room[:location][0] -= 1
    when 'R' then new_room[:location][0] += 1
    end

    q << new_room
  end
end
