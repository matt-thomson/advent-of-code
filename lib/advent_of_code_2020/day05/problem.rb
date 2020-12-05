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
    end
  end
end
