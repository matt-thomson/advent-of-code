# frozen_string_literal: true

module AdventOfCode2020
  module Day15
    class Sequence
      def initialize(starting_numbers)
        @starting_numbers = starting_numbers
      end

      def numbers(&block)
        return enum_for(:numbers) unless block

        @starting_numbers.each(&block)

        seen = @starting_numbers[0...-1].each_with_index.to_h
        last = @starting_numbers[-1]

        (@starting_numbers.length - 1...).each do |index|
          current = index - seen.fetch(last, index)
          yield current

          seen[last] = index
          last = current
        end
      end
    end
  end
end
