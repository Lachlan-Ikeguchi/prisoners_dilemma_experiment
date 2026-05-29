-- name: AGGRESSOR
-- starts aggressive, then mellows if opponent cooperates

function decide(round, history)
    if round <= 5 then
        return "defect"
    end
    
    local coop_count = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop_count = coop_count + 1 end
    end
    
    if coop_count >= #history * 0.7 then
        return "cooperate"
    else
        return "defect"
    end
end
