require 'digest'

def move(location, dir)
  res = location.dup

  case dir
  when 'U' then res[1] -= 1
  when 'D' then res[1] += 1
  when 'L' then res[0] -= 1
  when 'R' then res[0] += 1
  else raise "Invalid dir: #{dir}"
  end

  res
end

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

def verify(passcode, full_path)
  location = [0, 0]
  i = 0
  while i < full_path.size do
    p = full_path[0...i]
    dir = full_path[i]

    dirs = available_dirs(passcode, p, location)
    raise "#{dir} not available in #{dirs}" unless dirs.include?(dir)

    location = move(location, dir)

    if location[0] < 0 || 3 < location[0] ||
       location[1] < 0 || 3 < location[1]
      raise "Invalid location #{location}"
    end

    i += 1
  end
  raise "Not ending on [3, 3]: #{location}" unless location == [3, 3]
end

q = [{path: '', location: [0, 0]}]
passcode = 'edjrjqaa'
longest = -1

until q.empty?
  room = q.pop # DFS

  if room[:location] == [3, 3]
    # verify(passcode, room[:path])
    longest = [room[:path].size, longest].max
    next
  end

  dirs = available_dirs(passcode, room[:path], room[:location])
  next if dirs.empty?

  dirs.each do |dir|
    q << {
      path: room[:path] + dir,
      location: move(room[:location], dir),
    }
  end
end

puts longest
