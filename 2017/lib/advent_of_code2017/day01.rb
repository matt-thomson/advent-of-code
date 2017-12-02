# frozen_string_literal: true

module AdventOfCode2017
  module Day01
    def self.solve_part_one(input)
      digits = input.split(//).map(&:to_i)
      digits << digits[0]

      sum_matching_pairs(digits.each_cons(2))
    end

    def self.solve_part_two(input)
      digits = input.split(//).map(&:to_i)

      first = digits[0...(digits.length / 2)]
      second = digits[(digits.length / 2)..-1]

      sum_matching_pairs(first.zip(second)) * 2
    end

    def self.sum_matching_pairs(digits)
      digits.select { |x, y| x == y }
            .map { |pair| pair[0] }
            .inject(0) { |sum, x| sum + x }
    end
    private_class_method :sum_matching_pairs
  end
end
