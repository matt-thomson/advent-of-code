# frozen_string_literal: true

module AdventOfCode2020
  module Day07
    class Rule
      RULE_REGEX = /\A(?<colour>[a-z ]+) bags contain (?<conditions>.*).\Z/.freeze
      CONDITION_REGEX = /\A(?<count>\d+) (?<colour>[a-z ]+) bags?\Z/.freeze

      def self.parse(input)
        rule_matches = input.match(RULE_REGEX)
        conditions = parse_conditions(rule_matches[:conditions])

        new(rule_matches[:colour], conditions)
      end

      def self.parse_conditions(input)
        return {} if input == 'no other bags'

        input.split(', ').to_h do |c|
          match = c.match(CONDITION_REGEX)
          [match[:colour], match[:count].to_i]
        end
      end
      private_class_method :parse_conditions

      def initialize(colour, conditions)
        @colour = colour
        @conditions = conditions
      end

      attr_reader :colour, :conditions
    end
  end
end
