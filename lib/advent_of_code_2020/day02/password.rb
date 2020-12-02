# frozen_string_literal: true

module AdventOfCode2020
  module Day02
    class Password
      INPUT_REGEX = /\A(?<first>\d+)-(?<second>\d+) (?<char>[a-z]): (?<password>[a-z]+)\Z/.freeze

      def initialize(input)
        matches = input.match(INPUT_REGEX)

        @first = matches[:first].to_i
        @second = matches[:second].to_i
        @char = matches[:char]
        @password = matches[:password]
      end

      def valid_for_part_one?
        (@first..@second).cover?(@password.count(@char))
      end

      def valid_for_part_two?
        (@password[@first - 1] == @char) ^ (@password[@second - 1] == @char)
      end
    end
  end
end
