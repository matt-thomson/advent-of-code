# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    class Document
      REQUIRED_FIELDS = %w[byr iyr eyr hgt hcl ecl pid].freeze

      def initialize(input)
        @fields = input.split(/\s/).map { |field| field.split(':') }.to_h
      end

      def valid_passport?
        REQUIRED_FIELDS.all? { |key| @fields.include?(key) }
      end
    end
  end
end
