# frozen_string_literal: true

module AdventOfCode2017
  module Day04
    def self.solve_part_one(input)
      solve(input) { |word| word }
    end

    def self.solve_part_two(input)
      solve(input) { |word| word.chars.sort.join }
    end

    def self.solve(input, &block)
      input.split("\n")
           .map { |line| line.split.map(&block) }
           .select { |words| words.uniq == words }
           .count
    end
    private_class_method :solve
  end
end
