# frozen_string_literal: true

module AdventOfCode2020
  module Day02
    class Problem
      def initialize(path)
        @path = path
      end

      def part_one
        passwords = File.readlines(@path).map { |line| Password.new(line) }
        passwords.count(&:valid?)
      end
    end
  end
end
