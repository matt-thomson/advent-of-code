# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        documents = File.read(@path).split("\n\n").map { |input| Document.new(input) }
        documents.count(&:valid_passport?)
      end
    end
  end
end
