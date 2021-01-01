class MinHeap
  def initialize
    @heap = []
  end

  def insert(obj, priority)
    @heap << {
      obj: obj,
      priority: priority
    }

    push_up(@heap.size - 1)

    obj
  end

  def pop
    return nil if @heap.empty?
    return @heap.pop[:obj] if @heap.size == 1

    obj = @heap[0][:obj]

    # re-organize, pushing the last element downward
    @heap[0] = @heap.pop
    push_down(0)

    obj
  end

  # uses brute-force linear search for index of obj. If obj stored its index or
  # priority, this would be faster.
  def set_priority(obj, priority)
    i = @heap.index { |wrapper| wrapper[:obj] == obj }
    wrapper = @heap[i]
    return if wrapper[:priority] == priority

    wrapper[:priority] = priority
    i = push_up(i)
    push_down(i)
  end

  def to_s
    @heap.inspect
  end

  private

  def push_up(i)
    loop do
      return i if i == 0
      new_i = (i-1) / 2
      return i unless @heap[new_i][:priority] > @heap[i][:priority]
      @heap[i], @heap[new_i] = @heap[new_i], @heap[i]
      i = new_i
    end
  end

  def push_down(i)
    loop do
      a = 2*i + 1
      b = 2*i + 2

      return i unless @heap[a]

      min_child = a
      min_child = b if @heap[b] && @heap[b][:priority] < @heap[a][:priority]

      return i if @heap[i][:priority] < @heap[min_child][:priority]

      @heap[i], @heap[min_child] = @heap[min_child], @heap[i]
      i = min_child
    end
  end
end
