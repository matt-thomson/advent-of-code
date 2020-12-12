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

      def step(threshold, &neighbours)
        next_seats = @seats.map(&:dup)

        seats.each_with_index do |row, y|
          (0...row.length).each do |x|
            next_seats[y][x] = next_state(x, y, threshold, &neighbours)
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

      def next_state(x, y, threshold, &neighbours)
        occupied = occupied_neighbours(x, y, &neighbours)

        if seats[y][x] == :empty_seat && occupied.zero?
          :occupied_seat
        elsif seats[y][x] == :occupied_seat && occupied >= threshold
          :empty_seat
        else
          seats[y][x]
        end
      end

      def occupied_neighbours(x, y)
        yield(x, y, seats).count do |neighbour_x, neighbour_y|
          seats.fetch(neighbour_y, [])[neighbour_x] == :occupied_seat
        end
      end

      def self.parse_line(line)
        line.chars.map { |c| CHARACTERS.fetch(c) }
      end
      private_class_method :parse_line
    end
  end
end
