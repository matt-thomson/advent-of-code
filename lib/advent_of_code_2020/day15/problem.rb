# frozen_string_literal: true

require_relative 'sequence'

module AdventOfCode2020
  module Day15
    class Problem
      def initialize(*input)
        starting_numbers = input.map(&:to_i)
        @sequence = Sequence.new(starting_numbers)
      end

      def part_one
        solve(2020)
      end

      def part_two
        solve(30_000_000)
      end

      private

      def solve(index)
        @sequence.numbers.take(index).last
      end
    end
  end
end
