# frozen_string_literal: true

module AdventOfCode2020
  module Day02
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        solve(:valid_for_part_one?)
      end

      def part_two
        solve(:valid_for_part_two?)
      end

      private

      def solve(predicate)
        passwords = File.readlines(@path).map { |line| Password.new(line) }
        passwords.count(&predicate)
      end
    end
  end
end
