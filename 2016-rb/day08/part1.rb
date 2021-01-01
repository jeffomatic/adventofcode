input = File.open('./input') { |f| f.read }.strip.split("\n")

class Framebuffer
  def initialize(rows:, cols:)
    @grid = []
    rows.times do
      @grid << cols.times.map { false }
    end
  end

  def exec(ins)
    send(ins[:message], ins[:params])
  end

  def rect(rows:, cols:)
    rows.times do |i|
      cols.times do |j|
        @grid[i][j] = true
      end
    end
  end

  def rot_row(row:, n:)
    @grid[row].rotate!(-n)
  end

  def rot_col(col:, n:)
    set_col(col, col_vals(col).rotate(-n))
  end

  def count_on
    @grid.reduce(0) do |memo, row|
      memo += row.select{ |v| v }.count
      memo
    end
  end

  def to_s
    res = ''
    @grid.each do |row|
      row.each do |v|
        res += v ? '#' : '.'
      end
      res += "\n"
    end
    res
  end

  private

  def col_vals(j)
    @grid.reduce([]) { |memo, row| memo << row[j]; memo }
  end

  def set_col(j, vals)
    vals.each_with_index { |v, i| @grid[i][j] = v }
  end
end

def trim_prefix(s, prefix)
  s[prefix.size..-1]
end

def parse_instruction(ins)
  case
  when ins.start_with?('rect ')
    ins = trim_prefix(ins, 'rect ')
    cols, rows = ins.split('x').map(&:to_i)
    {
      message: :rect,
      params: {
        cols: cols,
        rows: rows,
      },
    }
  when ins.start_with?('rotate row y=')
    ins = trim_prefix(ins, 'rotate row y=')
    row, n = ins.split(' by ').map(&:to_i)
    {
      message: :rot_row,
      params: {
        row: row,
        n: n,
      },
    }
  when ins.start_with?('rotate column x=')
    ins = trim_prefix(ins, 'rotate column x=')
    col, n = ins.split(' by ').map(&:to_i)
    {
      message: :rot_col,
      params: {
        col: col,
        n: n,
      },
    }
  end
end

fb = Framebuffer.new(rows: 6, cols: 50)
input.each do |ins|
  fb.exec(parse_instruction(ins))
end

puts fb.count_on
puts fb.to_s

