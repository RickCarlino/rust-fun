#!/usr/bin/env ruby
#
# app_name      This is a startup script for use in /etc/init.d
#
# chkconfig:    2345 80 20
# description:  Description of program / service
APP_NAME = 'app_name'
case ARGV.first
when 'status':
status = 'stopped'
puts "#{APP_NAME} is #{status}"
when 'start':     # Do your thang
when 'stop':    # Do your thang
when 'restart':     # Do your thang
 end  unless %w{start stop restart status}.include? ARGV.first
puts "Usage: #{APP_NAME} {start|stop|restart}"
exit
end
