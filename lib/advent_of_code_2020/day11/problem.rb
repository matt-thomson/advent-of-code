# frozen_string_literal: true

require_relative 'waiting_room'

module AdventOfCode2020
  module Day11
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        solve(4) { |x, y, _seats| neighbours_part_one(x, y) }
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
        [x - 1, x, x + 1]
          .product([y - 1, y, y + 1])
          .reject { |neighbour_x, neighbour_y| neighbour_x == x && neighbour_y == y }
          .reject { |neighbour_x, neighbour_y| neighbour_x.negative? || neighbour_y.negative? }
      end
    end
  end
end
