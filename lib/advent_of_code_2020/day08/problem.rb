# frozen_string_literal: true

require_relative 'computer'

module AdventOfCode2020
  module Day08
    class Problem
      def initialize(path)
        @instructions = File
                        .readlines(path)
                        .map(&:split)
                        .map { |instruction, count| [instruction, count.to_i] }
      end

      def part_one
        computer = Computer.new(@instructions)
        computer.run!

        computer.accumulator
      end
    end
  end
end
