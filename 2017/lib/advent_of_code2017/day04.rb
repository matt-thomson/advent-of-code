# frozen_string_literal: true

module AdventOfCode2017
  module Day04
    def self.solve_part_one(input)
      input.split("\n")
           .map(&:split)
           .select { |words| words.uniq == words }
           .count
    end
  end
end
