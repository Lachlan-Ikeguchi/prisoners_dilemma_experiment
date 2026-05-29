-- name: BLUFFER
-- starts with cooperate, cooperate, defect to test waters

function decide(round, history)
    if round == 1 then return "cooperate" end
    if round == 2 then return "cooperate" end
    if round == 3 then return "defect" end
    
    if #history >= 3 then
        -- If opponent cooperated on round 3, they're exploitable
        if history[3] == "cooperate" then
            return "defect"
        end
    end
    
    return "cooperate"
end
