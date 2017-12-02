# frozen_string_literal: true

module AdventOfCode2017
  module Day02
    def self.solve_part_one(input)
      input.split("\n")
           .map { |line| score(line) }
           .inject(0) { |sum, x| sum + x }
    end

    def self.score(line)
      numbers = line.split.map(&:to_i)
      numbers.max - numbers.min
    end
    private_class_method :score
  end
end
