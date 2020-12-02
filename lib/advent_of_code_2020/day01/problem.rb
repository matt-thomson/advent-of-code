# frozen_string_literal: true

module AdventOfCode2020
  module Day01
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        solve(2)
      end

      def part_two
        solve(3)
      end

      private

      def solve(group_size)
        entries = File.readlines(@path).map(&:to_i)
        match = entries.combination(group_size).find { |group| group.sum == 2020 }

        match.inject(:*)
      end
    end
  end
end
