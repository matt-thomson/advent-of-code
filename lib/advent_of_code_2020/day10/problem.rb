# frozen_string_literal: true

module AdventOfCode2020
  module Day10
    class Problem
      def initialize(path)
        @adapters = File.readlines(path).map(&:to_i)
        @target = @adapters.max + 3
      end

      def part_one
        differences = Hash.new(0)

        (@adapters + [0, @target]).sort.each_cons(2) do |first, second|
          differences[second - first] += 1
        end

        differences.fetch(1) * differences.fetch(3)
      end

      def part_two
        arrangements = Hash.new(0)
        arrangements[0] = 1

        (@adapters + [@target]).sort.each do |adapter|
          arrangements[adapter] = (1..3).sum { |i| arrangements[adapter - i] }
        end

        arrangements[@target]
      end
    end
  end
end
