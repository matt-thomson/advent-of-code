# frozen_string_literal: true

module AdventOfCode2020
  module Day11
    class WaitingRoom
      CHARACTERS = {
        '.' => :floor,
        'L' => :empty_seat,
        '#' => :occupied_seat
      }.freeze
      private_constant :CHARACTERS

      def self.parse(input)
        seats = input.lines.map(&:strip).map { |line| parse_line(line) }
        new(seats)
      end

      def initialize(seats)
        @seats = seats
      end

      def step
        next_seats = @seats.map(&:dup)

        seats.each_with_index do |row, y|
          (0...row.length).each do |x|
            next_seats[y][x] = next_state(x, y)
          end
        end

        self.class.new(next_seats)
      end

      def to_s
        seats.map { |row| row.map { |c| CHARACTERS.key(c) }.join }.join("\n")
      end

      def count_occupied_seats
        seats.sum { |row| row.count { |seat| seat == :occupied_seat } }
      end

      attr_reader :seats

      private

      def next_state(x, y)
        if seats[y][x] == :empty_seat && occupied_neighbours(x, y).zero?
          :occupied_seat
        elsif seats[y][x] == :occupied_seat && occupied_neighbours(x, y) >= 4
          :empty_seat
        else
          seats[y][x]
        end
      end

      def occupied_neighbours(x, y)
        [x - 1, x, x + 1]
          .product([y - 1, y, y + 1])
          .reject { |neighbour_x, neighbour_y| neighbour_x == x && neighbour_y == y }
          .reject { |neighbour_x, neighbour_y| neighbour_x.negative? || neighbour_y.negative? }
          .count { |neighbour_x, neighbour_y| seats.fetch(neighbour_y, [])[neighbour_x] == :occupied_seat }
      end

      def self.parse_line(line)
        line.chars.map { |c| CHARACTERS.fetch(c) }
      end
      private_class_method :parse_line
    end
  end
end
