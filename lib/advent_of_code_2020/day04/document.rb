# frozen_string_literal: true

require_relative './validators/no_op_validator'

module AdventOfCode2020
  module Day04
    class Document
      VALIDATORS = {
        'byr' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'iyr' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'eyr' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'hgt' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'hcl' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'ecl' => AdventOfCode2020::Day04::Validators::NoOpValidator,
        'pid' => AdventOfCode2020::Day04::Validators::NoOpValidator
      }.freeze

      def initialize(input)
        @fields = input.split(/\s/).map { |field| field.split(':') }.to_h
      end

      def valid_structure?
        VALIDATORS.all? { |key, _| @fields.include?(key) }
      end
    end
  end
end
