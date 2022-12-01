# frozen_string_literal: true

require_relative 'waiting_room'

module AdventOfCode2020
  module Day11
    class Problem
      DIRECTIONS = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1]
      ].freeze

      def initialize(path)
        @path = path
      end

      def part_one
        solve(4) { |x, y, _seats| neighbours_part_one(x, y) }
      end

      def part_two
        solve(5) { |x, y, seats| neighbours_part_two(x, y, seats) }
      end

      private

      def solve(threshold, &neighbours)
        waiting_room = WaitingRoom.parse(File.read(@path))

        loop do
          next_waiting_room = waiting_room.step(threshold, &neighbours)
          break if next_waiting_room.seats == waiting_room.seats

          waiting_room = next_waiting_room
        end

        waiting_room.count_occupied_seats
      end

      def neighbours_part_one(x, y)
        DIRECTIONS
          .map { |dx, dy| [x + dx, y + dy] }
          .reject { |neighbour_x, neighbour_y| neighbour_x.negative? || neighbour_y.negative? }
      end

      def neighbours_part_two(x, y, seats)
        DIRECTIONS.map { |dx, dy| neighbour_part_two(x, y, dx, dy, seats) }.compact
      end

      def neighbour_part_two(x, y, dx, dy, seats)
        neighbour_x = x
        neighbour_y = y

        loop do
          neighbour_x += dx
          neighbour_y += dy

          return if neighbour_x.negative? || neighbour_y.negative?

          seat = seats.fetch(neighbour_y, [])[neighbour_x]
          return if seat.nil?

          return [neighbour_x, neighbour_y] if seat != :floor
        end
      end
    end
  end
end
