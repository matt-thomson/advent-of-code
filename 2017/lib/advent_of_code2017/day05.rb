# frozen_string_literal: true

module AdventOfCode2017
  module Day05
    def self.solve_part_one(input)
      solve(input) { |_offset| 1 }
    end

    def self.solve_part_two(input)
      solve(input) do |offset|
        if offset >= 3
          - 1
        else
          1
        end
      end
    end

    def self.solve(input)
      jumps = input.split.map(&:to_i)

      count = 0
      position = 0

      loop do
        break if position.negative?
        break if position >= jumps.length

        count += 1

        offset = jumps[position]
        jumps[position] += yield(offset)
        position += offset
      end

      count
    end
    private_class_method :solve
  end
end
