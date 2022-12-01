# frozen_string_literal: true

module AdventOfCode2020
  module Day12
    class Problem
      def initialize(path)
        @instructions = File.readlines(path).map { |line| [line[0], line[1..].to_i] }
      end

      def part_one
        position = [0, 0]
        direction = 'E'

        @instructions.each { |action, value| position, direction = step_part_one(position, direction, action, value) }

        position.sum(&:abs)
      end

      def part_two
        ship = [0, 0]
        waypoint = [10, 1]

        @instructions.each { |action, value| ship, waypoint = step_part_two(ship, waypoint, action, value) }

        ship.sum(&:abs)
      end

      private

      # rubocop:disable Metrics/AbcSize
      def step_part_one(position, direction, action, value)
        x, y = position

        {
          'N' => -> { [[x, y + value], direction] },
          'S' => -> { [[x, y - value], direction] },
          'E' => -> { [[x + value, y], direction] },
          'W' => -> { [[x - value, y], direction] },
          'L' => -> { [position, turn_left(direction, value / 90)] },
          'R' => -> { [position, turn_right(direction, value / 90)] },
          'F' => -> { step_part_one(position, direction, direction, value) }
        }.fetch(action).call
      end
      # rubocop:enable Metrics/AbcSize

      # rubocop:disable Metrics/AbcSize
      # rubocop:disable Metrics/MethodLength
      def step_part_two(ship, waypoint, action, value)
        ship_x, ship_y = ship
        waypoint_x, waypoint_y = waypoint

        {
          'N' => -> { [ship, [waypoint_x, waypoint_y + value]] },
          'S' => -> { [ship, [waypoint_x, waypoint_y - value]] },
          'E' => -> { [ship, [waypoint_x + value, waypoint_y]] },
          'W' => -> { [ship, [waypoint_x - value, waypoint_y]] },
          'L' => -> { [ship, rotate_left(waypoint, value / 90)] },
          'R' => -> { [ship, rotate_right(waypoint, value / 90)] },
          'F' => -> { [[ship_x + (waypoint_x * value), ship_y + (waypoint_y * value)], waypoint] }
        }.fetch(action).call
      end
      # rubocop:enable Metrics/AbcSize
      # rubocop:enable Metrics/MethodLength

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

      def rotate_left(position, times)
        return position if times.zero?

        x, y = position
        rotate_left([-y, x], times - 1)
      end

      def rotate_right(position, times)
        return position if times.zero?

        x, y = position
        rotate_right([y, -x], times - 1)
      end
    end
  end
end
