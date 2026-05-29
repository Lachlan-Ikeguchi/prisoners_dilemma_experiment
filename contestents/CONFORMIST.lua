-- name: CONFORMIST
-- matches the opponent's most frequent move in the last 5 rounds

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local coop = 0
    local defect = 0
    local start = math.max(1, #history - 4)
    
    for i = start, #history do
        if history[i] == "cooperate" then coop = coop + 1
        else defect = defect + 1 end
    end
    
    if coop > defect then return "cooperate"
    elseif defect > coop then return "defect"
    else return "cooperate" end
end
