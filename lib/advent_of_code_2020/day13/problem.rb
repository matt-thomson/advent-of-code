# frozen_string_literal: true

module AdventOfCode2020
  module Day13
    class Problem
      def initialize(path)
        input = File.readlines(path)

        @timestamp = input[0].to_i
        @buses = input[1]
                 .strip
                 .split(',')
                 .each_with_index
                 .reject { |bus, _| bus == 'x' }
                 .map { |bus, index| [bus.to_i, index] }
      end

      def part_one
        first_bus = @buses.min_by { |bus, _| minutes_until_bus(bus) }[0]
        minutes_until_bus(first_bus) * first_bus
      end

      private

      def minutes_until_bus(bus)
        bus - (@timestamp % bus)
      end
    end
  end
end
