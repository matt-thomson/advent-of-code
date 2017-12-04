# frozen_string_literal: true

module AdventOfCode2017
  module Day01
    def self.solve_part_one(input)
      solve(input) { |digits| (digits + [digits[0]]).each_cons(2) }
    end

    def self.solve_part_two(input)
      solve(input) do |digits|
        first = digits[0...(digits.length / 2)]
        second = digits[(digits.length / 2)..-1]

        first.zip(second)
      end * 2
    end

    def self.solve(input)
      digits = input.chars.map(&:to_i)
      pairs = yield digits

      pairs.select { |x, y| x == y }
           .map { |pair| pair[0] }
           .inject(0) { |sum, x| sum + x }
    end
    private_class_method :solve
  end
end
