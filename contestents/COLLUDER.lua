-- name: COLLUDER
-- tries to establish mutual cooperation by starting with cooperate, defect, cooperate

function decide(round, history)
    if round == 1 then return "cooperate" end
    if round == 2 then return "defect" end
    if round == 3 then return "cooperate" end
    
    -- After initial pattern, if opponent followed the pattern, keep cooperating
    if #history >= 3 then
        if history[1] == "cooperate" and history[2] == "defect" and history[3] == "cooperate" then
            return "cooperate"
        end
    end
    
    -- Otherwise mirror
    return history[#history]
end
