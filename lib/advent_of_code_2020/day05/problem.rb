# frozen_string_literal: true

require_relative 'boarding_pass'

module AdventOfCode2020
  module Day05
    class Problem
      def initialize(path)
        @boarding_passes = File.readlines(path).map { |input| BoardingPass.new(input) }
      end

      def part_one
        @boarding_passes.map(&:seat_id).max
      end

      def part_two
        seat_ids = @boarding_passes.map(&:seat_id)
        all_seats = (seat_ids.min..seat_ids.max).to_a
        missing_seats = all_seats - seat_ids

        missing_seats.first
      end
    end
  end
end
