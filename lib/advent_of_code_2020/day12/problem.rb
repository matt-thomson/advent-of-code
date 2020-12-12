# frozen_string_literal: true

module AdventOfCode2020
  module Day12
    class Problem
      def initialize(path)
        @instructions = File.readlines(path).map { |line| [line[0], line[1..].to_i] }
      end

      def part_one
        x = 0
        y = 0
        direction = 'E'

        @instructions.each { |action, value| x, y, direction = step(x, y, direction, action, value) }

        x.abs + y.abs
      end

      private

      # rubocop:disable Metrics/AbcSize
      def step(x, y, direction, action, value)
        {
          'N' => -> { [x, y + value, direction] },
          'S' => -> { [x, y - value, direction] },
          'E' => -> { [x + value, y, direction] },
          'W' => -> { [x - value, y, direction] },
          'L' => -> { [x, y, turn_left(direction, value / 90)] },
          'R' => -> { [x, y, turn_right(direction, value / 90)] },
          'F' => -> { step(x, y, direction, direction, value) }
        }.fetch(action).call
      end
      # rubocop:enable Metrics/AbcSize

      def turn_left(direction, times)
        return direction if times.zero?

        after = { 'N' => 'W', 'E' => 'N', 'S' => 'E', 'W' => 'S' }.fetch(direction)

        turn_left(after, times - 1)
      end

      def turn_right(direction, times)
        return direction if times.zero?

        after = { 'N' => 'E', 'E' => 'S', 'S' => 'W', 'W' => 'N' }.fetch(direction)

        turn_right(after, times - 1)
      end
    end
  end
end
