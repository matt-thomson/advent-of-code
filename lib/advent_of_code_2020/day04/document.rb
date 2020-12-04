# frozen_string_literal: true

require_relative './validators/birth_year'
require_relative './validators/expiration_year'
require_relative './validators/eye_colour'
require_relative './validators/hair_colour'
require_relative './validators/height'
require_relative './validators/issue_year'
require_relative './validators/passport_id'

module AdventOfCode2020
  module Day04
    class Document
      VALIDATORS = {
        'byr' => AdventOfCode2020::Day04::Validators::BirthYear,
        'iyr' => AdventOfCode2020::Day04::Validators::IssueYear,
        'eyr' => AdventOfCode2020::Day04::Validators::ExpirationYear,
        'hgt' => AdventOfCode2020::Day04::Validators::Height,
        'hcl' => AdventOfCode2020::Day04::Validators::HairColour,
        'ecl' => AdventOfCode2020::Day04::Validators::EyeColour,
        'pid' => AdventOfCode2020::Day04::Validators::PassportID
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
