# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    class Problem
      def initialize(path)
        @documents = File.read(path).split("\n\n").map { |input| Document.new(input) }
      end

      def part_one
        @documents.count(&:valid_structure?)
      end

      def part_two
        @documents.count(&:valid?)
      end
    end
  end
end
