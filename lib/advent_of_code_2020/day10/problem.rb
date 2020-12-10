# frozen_string_literal: true

module AdventOfCode2020
  module Day10
    class Problem
      def initialize(path)
        @adapters = File.readlines(path).map(&:to_i)
      end

      def part_one
        differences = Hash.new(0)

        (@adapters + [0, @adapters.max + 3]).sort.each_cons(2) do |first, second|
          differences[second - first] += 1
        end

        differences.fetch(1) * differences.fetch(3)
      end
    end
  end
end
