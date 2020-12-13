# frozen_string_literal: true

module AdventOfCode2020
  module Day13
    class Problem
      def initialize(path)
        input = File.readlines(path)

        @timestamp = input[0].to_i
        @buses = input[1].strip.split(',').reject { |x| x == 'x' }.map(&:to_i)
      end

      def part_one
        bus = @buses.min_by { |x| minutes_until_bus(x) }
        minutes_until_bus(bus) * bus
      end

      private 

      def minutes_until_bus(bus)
        bus - (@timestamp % bus)
      end
    end
  end
end
