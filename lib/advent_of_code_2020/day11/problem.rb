# frozen_string_literal: true

require_relative 'waiting_room'

module AdventOfCode2020
  module Day11
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        waiting_room = WaitingRoom.parse(File.read(@path))

        loop do
          next_waiting_room = waiting_room.step
          break if next_waiting_room.seats == waiting_room.seats

          waiting_room = next_waiting_room
        end

        waiting_room.count_occupied_seats
      end
    end
  end
end
