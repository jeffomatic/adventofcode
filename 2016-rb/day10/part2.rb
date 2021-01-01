input = File.open('./input') { |f| f.read }.strip.split("\n")

chips_to_bots = {}
bots_to_low_bots = {}
bots_to_low_outputs = {}
bots_to_high_bots = {}
bots_to_high_outputs = {}
bot_ids = []

input.each do |command|
  command.match /value (\d+) goes to bot (\d+)/ do |m|
    chip = m[1].to_i
    bot = m[2].to_i
    chips_to_bots[chip] = bot
    bot_ids << bot
  end

  command.match /bot (\d+) gives low to ((bot)|(output)) (\d+) and high to ((bot)|(output)) (\d+)/ do |m|
    bot = m[1].to_i
    low_bot = m[2] == 'bot'
    low = m[5].to_i
    high_bot = m[6] == 'bot'
    high = m[9].to_i

    bot_ids << bot

    if low_bot
      bots_to_low_bots[bot] = low
      bot_ids << low
    else
      bots_to_low_outputs[bot] = low
    end

    if high_bot
      bots_to_high_bots[bot] = high
      bot_ids << high
    else
      bots_to_high_outputs[bot] = high
    end
  end
end

payloads = {}
bot_ids.sort.uniq.each { |id| payloads[id] = [] }

full_bots = {}
chips_to_bots.each do |chip, bot|
  payloads[bot] << chip
  full_bots[bot] = payloads[bot] if payloads[bot].size == 2
end

outputs = {}

until full_bots.empty?
  full_bots.each do |bot, payload|
    low = payload.min
    high = payload.max

    if bots_to_low_bots[bot]
      if payloads[bots_to_low_bots[bot]].size < 2
        payloads[bots_to_low_bots[bot]] << low
        payload.reject! { |chip| chip == low }
      end
    elsif bots_to_low_outputs[bot]
      outputs[bots_to_low_outputs[bot]] ||= []
      outputs[bots_to_low_outputs[bot]] << low
      payload.reject! { |chip| chip == low }
    end

    if bots_to_high_bots[bot]
      if payloads[bots_to_high_bots[bot]].size < 2
        payloads[bots_to_high_bots[bot]] << high
        payload.reject! { |chip| chip == high }
      end
    elsif bots_to_high_outputs[bot]
      outputs[bots_to_high_outputs[bot]] ||= []
      outputs[bots_to_high_outputs[bot]] << high
      payload.reject! { |chip| chip == high }
    end
  end

  full_bots = {}
  payloads.each do |bot, payload|
    full_bots[bot] = payload if payload.size == 2
  end
end

puts outputs[0].first * outputs[1].first * outputs[2].first
