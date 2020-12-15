# frozen_string_literal: true

require_relative 'sequence'

module AdventOfCode2020
  module Day15
    class Problem
      def initialize(*input)
        @starting_numbers = input.map(&:to_i)
      end

      def part_one
        sequence = Sequence.new(@starting_numbers)
        sequence.numbers.take(2020).last
      end
    end
  end
end
