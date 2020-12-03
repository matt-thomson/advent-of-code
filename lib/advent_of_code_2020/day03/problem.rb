# frozen_string_literal: true

module AdventOfCode2020
  module Day03
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        forest = Forest.new(@path)
        (0...forest.height).count { |y| forest.tree?(y * 3, y) }
      end
    end
  end
end
