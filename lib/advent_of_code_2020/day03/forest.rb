# frozen_string_literal: true

module AdventOfCode2020
  module Day03
    class Forest
      def initialize(input)
        @data = File.readlines(input).map(&:strip).map(&:chars)
        @width = @data.first.length
        @height = @data.length
      end

      def tree?(x, y)
        @data[y][x % @width] == '#'
      end

      attr_reader :height
    end
  end
end
