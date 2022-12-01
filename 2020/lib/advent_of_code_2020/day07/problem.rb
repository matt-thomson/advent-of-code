# frozen_string_literal: true

require_relative 'rule'

require 'set'

module AdventOfCode2020
  module Day07
    class Problem
      def initialize(path)
        @rules = File.readlines(path)
                     .map { |line| Rule.parse(line) }
                     .map { |rule| [rule.colour, rule] }
                     .to_h
      end

      def part_one
        allowed = Set.new
        to_check = ['shiny gold']

        until to_check.empty?
          colour = to_check.shift

          containers = @rules.values.select { |rule| rule.conditions.include?(colour) }.map(&:colour)
          new_containers = containers.reject { |container| allowed.include?(container) }

          allowed += new_containers
          to_check += new_containers
        end

        allowed.count
      end

      def part_two
        total_bags('shiny gold')
      end

      private

      def total_bags(colour)
        @rules.fetch(colour).conditions.sum do |inner_colour, count|
          (total_bags(inner_colour) + 1) * count
        end
      end
    end
  end
end
