# frozen_string_literal: true

module AdventOfCode2020
  module Day09
    class Problem
      def initialize(path, preamble: 25)
        @input = File.readlines(path).map(&:to_i)
        @preamble = preamble
      end

      def part_one
        @input.each_cons(@preamble + 1).find do |slice|
          slice[0...@preamble].combination(2).none? do |combination|
            combination.sum == slice.last
          end
        end.last
      end

      def part_two
        target = part_one

        (0..@input.length).each do |first|
          total = 0
          (first..@input.length).each do |last|
            total += @input[last]
            break if total > target

            return @input[first..last].min + @input[first..last].max if total == target
          end
        end
      end
    end
  end
end
