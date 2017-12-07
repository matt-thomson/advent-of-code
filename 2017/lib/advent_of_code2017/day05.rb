# frozen_string_literal: true

module AdventOfCode2017
  module Day05
    def self.solve_part_one(input)
      jumps = input.split.map(&:to_i)

      count = 0
      position = 0

      loop do
        break if position.negative?
        break if position >= jumps.length

        count += 1

        old_jumps = jumps[position]
        jumps[position] += 1
        position += old_jumps
      end

      count
    end
  end
end
