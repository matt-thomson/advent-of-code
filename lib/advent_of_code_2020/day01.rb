module AdventOfCode2020
  class Day01
    def initialize(path)
      @path = path
    end

    def part_one
      entries = File.read(@path).lines.map(&:strip).map(&:to_i)

      first, second = entries.
        product(entries).
        find { |first, second| first + second == 2020 }
      
      first * second
    end
  end
end
