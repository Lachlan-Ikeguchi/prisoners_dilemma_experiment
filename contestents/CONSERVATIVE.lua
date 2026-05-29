-- name: CONSERVATIVE
-- sticks to safe choices

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local coop = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop = coop + 1 end
    end
    
    if coop / #history > 0.7 then
        return "cooperate"
    else
        return "defect"
    end
end
