# frozen_string_literal: true

module AdventOfCode2020
  module Day06
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        solve { |group| group.gsub(/\s/, '').chars.uniq.count }
      end

      def part_two
        solve { |group| group.lines.map(&:strip).map(&:chars).inject(:&).count }
      end

      private

      def solve(&block)
        File.read(@path).split("\n\n").sum(&block)
      end
    end
  end
end
