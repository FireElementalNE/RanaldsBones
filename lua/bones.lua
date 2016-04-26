require "io"
function roll_dice (cutoff)
	return math.random() < cutoff	
end
function check_args(t, g, b)
	if(t) then
		if(t > 3 or t < 0) then
			print("Incorrect number of tomes")
			do return end
		end
	end
	if(g) then
		if(g > 2 or g < 0) then
			print("Incorrect number of grimoires")
			do return end
		end
	end
	if(b) then
		if(b > 2 or b < 0) then
			print("Incorrect number of bonus dice")
			do return end
		end
	end
end
math.randomseed(os.time())
tome_cutoff = 2/3
regular_cutoff = 1/3
bonus_cutoff = 1/2
tomes = 0
grims = 0
bonus = 0
reg = 7
if(arg[1]) then
	tomes = tonumber(arg[1])
end
if(arg[2]) then
	grims = tonumber(arg[2])
end
if(arg[3]) then
	bonus = tonumber(arg[3])
end
reg = reg - tomes - grims - bonus
print("Dice pool is:")
print(" " .. tomes .. " Tome(s)")
print(" " .. grims .. " Grimoires(s)")
print(" " .. bonus .. " Bonus Dice")
print(" " .. reg .. " Regular Dice")
print("-------------------")
succ_count = 0
for i=1,tomes do
	if(roll_dice(tome_cutoff)) then
		print("> S for Tome #" .. i)
		succ_count = succ_count + 1
	else
		print("> F for Tome #" .. i)
	end
end
for i=1,grims do
	print("> S for Grimoire #" .. i)
	succ_count = succ_count + 1
end
for i=1,bonus do
	if(roll_dice(bonus_cutoff)) then
		print("> S for Bonus Dice #" .. i)
		succ_count = succ_count + 1
	else
		print("> F for Bonus Dice #" .. i)
	end
end
for i=1,reg do
	if(roll_dice(regular_cutoff)) then
		print("> S for Regular Dice #" .. i)
		succ_count = succ_count + 1
	else
		print("> F for Regular Dice #" .. i)
	end
end
print("-------------------")
print("Total is " .. succ_count)
