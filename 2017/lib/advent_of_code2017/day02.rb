# frozen_string_literal: true

module AdventOfCode2017
  module Day02
    def self.solve_part_one(input)
      solve(input) { |numbers| score_part_one(numbers) }
    end

    def self.solve_part_two(input)
      solve(input) { |numbers| score_part_two(numbers) }
    end

    def self.solve(input)
      input.split("\n")
           .map { |line| line.split.map(&:to_i) }
           .map { |numbers| yield(numbers) }
           .inject(0) { |sum, x| sum + x }
    end
    private_class_method :solve

    def self.score_part_one(numbers)
      numbers.max - numbers.min
    end
    private_class_method :score_part_one

    def self.score_part_two(numbers)
      x, y = numbers.product(numbers)
                    .reject { |x, y| x <= y }
                    .find { |x, y| (x % y).zero? }

      x / y
    end
    private_class_method :score_part_two
  end
end
