# frozen_string_literal: true

require_relative 'forest'

module AdventOfCode2020
  module Day03
    class Problem
      SLOPES = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2]
      ].freeze

      def initialize(path)
        @forest = Forest.new(path)
      end

      def part_one
        solve(3, 1)
      end

      def part_two
        SLOPES.map { |right, down| solve(right, down) }.inject(:*)
      end

      private

      def solve(right, down)
        num_steps = @forest.height / down
        (0...num_steps).count { |y| @forest.tree?(y * right, y * down) }
      end
    end
  end
end
